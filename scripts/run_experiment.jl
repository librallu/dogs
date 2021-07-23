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

# """
# concat paths a and b (both should be relative)
# if remove_last=true: remove b last path element
# """
# function concat_paths(a,b,remove_last=false)
#     res = a
#     c = splitpath(c)
#     if remove_last
#         c = pop!(c)
#     end
#     for d in c

#     end
#     return normpath(a+b)
# end

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
    parsed_args = parse_commandline()
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
    instances = read_csv(common["instance_csv_filename"])
    # generate and execute commands (cartesian product on instances, algos with params)
    # generate analysis
end
main()
