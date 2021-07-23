#!/usr/bin/env julia
using ArgParse
using TOML
using CSV
using Crayons
using Crayons.Box
using IterTools
using Dates

function read_configuration(configuration_filename)
    return TOML.parsefile(configuration_filename)
end

function read_csv(csv_filename)
    return CSV.File(csv_filename)
end

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

function common_variables(configuration, configuration_filename)
    instance_csv_filename = configuration["instance_filenames"]
    if !isabspath(instance_csv_filename)
        instance_csv_filename = abspath(
            joinpath(configuration_filename,"..",instance_csv_filename)
        )
    end
    experiment_name = configuration["experiment_name"]
    csv_instances_root = abspath(joinpath(instance_csv_filename, ".."))
    date_id = Dates.format(Dates.now(), "yyyy_mm_dd")
    output_directory = abspath(
        joinpath(configuration_filename,"..",configuration["output_prefix"])
    )
    return Dict(
        "instance_csv_filename" => instance_csv_filename,
        "experiment_name" => experiment_name,
        "csv_instances_root" => csv_instances_root,
        "date_id" => date_id,
        "output_directory" => output_directory
    )
end

function main()
    println(YELLOW_FG("GENRATING EXPERIMENTS..."))
    # read command line
    # parsed_args = parse_commandline()
    # debug comment (otherwise JIT compilation is way too slow!)
    parsed_args = Dict(
        "configuration" => "scripts/examples/test_experiment.toml",
        "debug" => "true"
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
    println(YELLOW_FG("READING CSV ($(common["instance_csv_filename"]))..."))
    # instances = read_csv(configuration["instance_filenames"])
    instances_csv = read_csv(common["instance_csv_filename"])
    insts = zip(instances_csv.path, instances_csv.time_limit)
    # generate and execute commands (cartesian product on instances, algos with params)
    println(YELLOW_FG("RUNNING EXPERIMENTS..."))
    solver_variants = []
    for solver in keys(configuration["solvers"])
        # TODO recursively reads the solver configuration and generates configurations
        println(solver)
    end
    # generate analysis
end
main()
