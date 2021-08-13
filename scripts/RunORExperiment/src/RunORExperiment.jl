module RunORExperiment

using ArgParse
using Crayons
using Crayons.Box

include("PerformExperiment.jl")
include("BuildAnalysis.jl")


""" function to parse the command line"""
function parse_commandline()
    s = ArgParseSettings(
        description="runs an experiment defined by a .toml file"
    )
    @add_arg_table s begin
        "--configuration"
            help = "configuration file (.toml)"
            required = true
            arg_type = String
        "--analysis_only"
            help = "only performs the analysis (requires the directory path of the computed experiment)"
            arg_type = String
        "--debug"
            help = "if set, only prints the commands instead of executing them"
            action = :store_true
        "--fallback_run"
            help = "if set, runs only the failed tests with lower number of threads"
            arg_type = String
    end
    return parse_args(s)
end


function main()
    PerformExperiment.tsp_check()
    println(YELLOW_FG("GENRATING EXPERIMENTS..."))
    ### read command line
    parsed_args = parse_commandline()
    ### debug comment (otherwise JIT compilation is way too slow!)
    # parsed_args = Dict(
    #     # "configuration" => "../examples/test_flowtime.json",
    #     "configuration" => "../../../dogs-pfsp/experiments/flowtime.experiment.json",
    #     # "configuration" => "../../../dogs-pfsp/experiments/taillard_makespan.experiment.json",
    #     # "configuration" => "../../../dogs-pfsp/experiments/vfr_large_makespan.experiment.json",
    #     "debug" => true,
    #     "analysis_only" => "../../../dogs-pfsp/experiments/flowtime_2021_07_27/"
    #     # "analysis_only" => "../../../dogs-pfsp/experiments/taillard_makespan_2021_07_29/",
    #     # "analysis_only" => "../../../dogs-pfsp/experiments/vfr_large_makespan_2021_07_29/"
    #     # "fallback_run" => "../../../dogs-pfsp/experiments/taillard_makespan_2021_07_29/"
    #     # "fallback_run" => nothing
    # )
    ###
    configuration_filename = abspath(parsed_args["configuration"])
    is_debug = parsed_args["debug"]
    analysis_only = ""
    if "analysis_only" in keys(parsed_args)
        analysis_only = parsed_args["analysis_only"]
    end
    fallback_run = nothing
    if "fallback_run" in keys(parsed_args)
        fallback_run = parsed_args["fallback_run"]
    end
    pad = 20
    println(rpad(" configuration:", pad, " ")*configuration_filename)
    println(rpad(" debug:", pad, " ")*string(is_debug))
    println(rpad(" analysis only:", pad, " ")*string(analysis_only))
    println(rpad(" fallback run:", pad, " ")*string(fallback_run))
    # read experiment .toml file
    println(YELLOW_FG("READING EXPERIMENT FILE ($(configuration_filename))..."))
    configuration = PerformExperiment.read_configuration(configuration_filename)
    pad = 25
    common = PerformExperiment.experiment_variables(configuration, configuration_filename, analysis_only, fallback_run)
    for i in keys(common)
        println(rpad(i*":", pad, " ")*common[i])
    end
    # read instance .csv file
    println(YELLOW_FG("SETTING UP TSP..."))
    PerformExperiment.tsp_set(configuration["nb_parallel_tasks"])
    println(YELLOW_FG("READING CSV ($(common["instance_csv_filename"]))..."))
    # instances = read_csv(configuration["instance_filenames"])
    instances_csv = PerformExperiment.read_csv(common["instance_csv_filename"])
    # generate and execute commands (cartesian product on instances, algos with params)
    println(YELLOW_FG("RUNNING EXPERIMENTS..."))
    solver_variants = PerformExperiment.compute_solver_variants(configuration, configuration_filename)
    println(YELLOW_FG("CREATING OUTPUT DIR $(common["output_directory"])..."))
    mkpath(common["output_directory"])
    println("$(common["output_directory"])")
    mkpath(common["output_directory"]*"/solver_results/")
    mkpath(common["output_directory"]*"/analysis/")
    # for each solver and instance, build the command to run
    solver_variant_with_instance = PerformExperiment.compute_solver_with_instance(
        solver_variants, common, instances_csv
    )
    # run each experiment if not analysis only
    if analysis_only == "" && fallback_run === nothing
        for experiment_id in keys(solver_variant_with_instance)
            command = solver_variant_with_instance[experiment_id]["command"]
            if is_debug
                println(command)
            else
                println("running $command ...")
                run(`sh -c $command`)
            end
        end
    end
    # when the solvers finished, generate analysis
    println(YELLOW_FG("WAITING FOR THE SOLVERS TO FINISH..."))
    PerformExperiment.tsp_wait()
    println(YELLOW_FG("GENERATING ANALYSIS..."))
    invalid_experiments = []
    # check that all tests have been correctly executed (all files are present)
    for k in keys(solver_variant_with_instance)
        if ! isfile("$(solver_variant_with_instance[k]["output_file_prefix"]).stats.json")
            push!(invalid_experiments, k)
            if fallback_run === nothing
                println(RED_FG("WARNING: non-existing result $(solver_variant_with_instance[k]["output_file_prefix"])"))
            end
        end
    end
    if fallback_run !== nothing
        println(GREEN_FG("$(length(invalid_experiments)) tests to run again"))
        fallback_threads = configuration["fallback_threads"]
        println("setting $(fallback_threads) threads")
        PerformExperiment.tsp_set(fallback_threads)
        for experiment_id in invalid_experiments
            command = solver_variant_with_instance[experiment_id]["command"]
            if is_debug
                println(command)
            else
                run(`sh -c $command`)
            end
        end
    end
    # read output files and populate solver_variant_with_instance
    for k in keys(solver_variant_with_instance)
        solver_variant_with_instance[k]["stats"] = PerformExperiment.read_performance_stats(solver_variant_with_instance[k]["output_file_prefix"]*".stats.json")
    end
    # generate analysis
    BuildAnalysis.build_analysis(
        configuration, common, instances_csv, solver_variants, solver_variant_with_instance
    )
end
main()


end # module
