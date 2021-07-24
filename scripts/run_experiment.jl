#!/usr/bin/env julia
using ArgParse
using JSON
using CSV
using Crayons
using Crayons.Box
using IterTools
using Dates

""" reads JSON experiment file """
function read_configuration(configuration_filename)
    res = Dict()
    open(configuration_filename, "r") do f
        res=JSON.parse(read(f,String))
    end
    return res
end

""" reads the CSV instance file """
function read_csv(csv_filename)
    return CSV.File(csv_filename)
end

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
        "--debug"
            help = "if set, only prints the commands instead of executing them"
            action = :store_true
    end
    return parse_args(s)
end

""" defines various common variables among the solvers """
function common_variables(configuration, configuration_filename)
    instance_csv_filename = configuration["instance_list"]
    if !isabspath(instance_csv_filename)
        instance_csv_filename = abspath(
            joinpath(configuration_filename,"..",instance_csv_filename)
        )
    end
    experiment_name = configuration["experiment_name"]
    csv_instances_root = abspath(joinpath(instance_csv_filename, ".."))
    date_id = Dates.format(Dates.now(), "yyyy_mm_dd")
    output_directory = abspath(
        joinpath(
            configuration_filename,"..",
            configuration["output_prefix"],
            "$(experiment_name)_$(date_id)/"
        )
    )
    return Dict(
        "instance_csv_filename" => instance_csv_filename,
        "experiment_name" => experiment_name,
        "csv_instances_root" => csv_instances_root,
        "date_id" => date_id,
        "output_directory" => output_directory
    )
end

function tsp_check()
    try 
        run(`which tsp`)
    catch ex
        println(RED_FG("ERROR: IS tsp INSTALLED ON THE MACHINE?"))
        exit(1)
    end
end

function tsp_set(nb_parallel::Int)
    run(`tsp -S 1`)
    for i in 1:1:200
        try
            run(`tsp -k`)    
        catch e
            break
        end
    end
    run(`tsp -K`)
    run(`tsp /bin/true`)
    run(`tsp -S $nb_parallel`)
end

# """ executes a shell command and returns its stdout as a string."""
# function cmd_run_and_get_stdout(cmd::Cmd)
#     out = Pipe()
#     err = Pipe()
#     process = run(pipeline(ignorestatus(cmd), stdout=out, stderr=err))
#     close(out.in)
#     close(err.in)
#     return String(read(out))
# end

""" waits the last job to end."""
function tsp_wait()
    run(`tsp -w`)
end

function main()
    tsp_check()
    println(YELLOW_FG("GENRATING EXPERIMENTS..."))
    ### read command line
    # parsed_args = parse_commandline()
    # debug comment (otherwise JIT compilation is way too slow!)
    parsed_args = Dict(
        "configuration" => "examples/test_flowtime.json",
        "debug" => false
    )
    ###
    configuration_filename = abspath(parsed_args["configuration"])
    is_debug = parsed_args["debug"]
    pad = 20
    println(rpad(" configuration:", pad, " ")*configuration_filename)
    println(rpad(" debug:", pad, " ")*string(is_debug))
    # read experiment .toml file
    println(YELLOW_FG("READING EXPERIMENT FILE ($(configuration_filename))..."))
    configuration = read_configuration(configuration_filename)
    pad = 25
    common = common_variables(configuration, configuration_filename)
    for i in keys(common)
        println(rpad(i*":", pad, " ")*common[i])
    end
    # read instance .csv file
    println(YELLOW_FG("SETTING UP TSP..."))
    tsp_set(configuration["nb_parallel_tasks"])
    println(YELLOW_FG("READING CSV ($(common["instance_csv_filename"]))..."))
    # instances = read_csv(configuration["instance_filenames"])
    instances_csv = read_csv(common["instance_csv_filename"])
    insts = zip(instances_csv.path, instances_csv.time_limit)
    # generate and execute commands (cartesian product on instances, algos with params)
    println(YELLOW_FG("RUNNING EXPERIMENTS..."))
    solver_variants = []
    for solver in configuration["solvers"]
        solver_name = solver["name"]
        solver_path = solver["exe_path"]
        if !isabspath(solver_path)
            solver_path = abspath(
                joinpath(configuration_filename,"..",solver_path)
            )
        end
        param_names = []
        param_values = []
        for p in solver["params"]
            push!(param_names, p["name"])
            push!(param_values, p["values"])
        end
        for a in IterTools.product(param_values...)
            push!(solver_variants, Dict(
                "name" => solver_name,
                "path" => solver_path,
                "params" => collect(zip(param_names,a, param_values))
            ))
        end
    end
    println(YELLOW_FG("CREATING OUTPUT DIR $(common["output_directory"])..."))
    mkpath(common["output_directory"])
    mkpath(common["output_directory"]*"/solver_results/")
    mkpath(common["output_directory"]*"/analysis/")
    # for each solver and instance, build the command to run
    last_task_id = 0
    for solver_conf in solver_variants
        for inst in instances_csv
            command = "tsp $(solver_conf["path"])"
            instance_name = inst.name
            for arg in solver_conf["params"]
                if arg[1] != ""
                    command *= " --$(arg[1])"
                end
                command *= " $(arg[2])"
            end
            # replace patterns
            solver_params_compact = ""
            for v in solver_conf["params"]
                if length(v[3]) > 1
                    solver_params_compact *= "_$(v[2])"
                end
            end
            command = replace(command,
                "#{instance_path}"
                =>abspath(joinpath(common["instance_csv_filename"],"..",inst.path)),
            )
            command = replace(command,
                "#{time_limit}"
                =>inst.time_limit
            )
            command = replace(command,
                "#{file_prefix}"
                =>common["output_directory"]*"/solver_results/$(solver_conf["name"])$(solver_params_compact)_$(instance_name)"
            )
            if parsed_args["debug"]
                println(command)
            else
                run(`sh -c $command`)
                println(command)
            end
        end
    end
    println(YELLOW_FG("WAITING FOR THE SOLVERS TO FINISH..."))
    tsp_wait()
    # when the solvers finished, generate analysis
end
main()
