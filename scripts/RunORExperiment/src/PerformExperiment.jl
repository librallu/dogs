"""
Provides functions to run an experiment on the target machine
"""
module PerformExperiment

using JSON
using CSV
using IterTools
using Dates

""" reads the CSV instance file """
function read_csv(csv_filename)
    return CSV.File(csv_filename)
end

""" reads JSON experiment file """
function read_configuration(configuration_filename)
    open(configuration_filename, "r") do f
        res = JSON.parse(read(f,String))
        res["filename"] = configuration_filename
        res
    end
end

"""
reads a JSON file describing the algorithm performance statistics.
"""
function read_performance_stats(filename::String)
    open(filename, "r") do f
        return JSON.parse(read(f,String))
    end
end


"""
from the instance list, build a dictionnary:
 - keys: instance classes names
 - values: list of instances belonging to the class of the key
"""
function build_instance_classes(instances_csv)
    instance_classes = Dict() # instance class -> vector of instance data
    for inst in instances_csv
        if inst.class_name in keys(instance_classes)
            push!(instance_classes[inst.class_name], inst)
        else
            instance_classes[inst.class_name] = [inst]
        end
    end
    return instance_classes
end


""" defines various common variables among the solvers """
function experiment_variables(configuration, configuration_filename, analysis_only)
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
    if analysis_only !== nothing
        output_directory = string(analysis_only)
    end
    return Dict(
        "instance_csv_filename" => instance_csv_filename,
        "experiment_name" => experiment_name,
        "csv_instances_root" => csv_instances_root,
        "date_id" => date_id,
        "output_directory" => output_directory
    )
end


""" gets the list of solver variants.
    ∀ solvers, cartesian product of each possible parameter configuration
    a solver variant includes its name, path, and parameter list
"""
function compute_solver_variants(configuration, configuration_filename)
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
    return solver_variants
end


"""
computes a solver with instance for each combination of solver variant and instance.
Runs it.
The return is a Dictionnary matching a String id "solver+params+id".
Each value includes the command to run the test, the instance name
and other useful information.
"""
function compute_solver_with_instance(solver_variants, common, instances_csv)
    solver_variant_with_instance = Dict()
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
            solver_conf["solver_params_compact"] = solver_params_compact
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
            id = "$(solver_conf["name"])$(solver_params_compact)_$(instance_name)"
            solver_conf["id"] = "$(solver_conf["name"])$(solver_params_compact)"
            solver_variant_with_instance[id] = Dict(
                "command" => command,
                "instance_name" => instance_name,
                "solver_conf" => solver_conf["params"],
                "output_file_prefix" => common["output_directory"]*"/solver_results/$(id)"
            )
        end
    end
    solver_variant_with_instance
end


""" checks that the task-spooler exists on the system """
function tsp_check()
    try 
        run(`which tsp`)
    catch _
        println(RED_FG("ERROR: IS tsp INSTALLED ON THE MACHINE?"))
        exit(1)
    end
end

""" set the number of parallel tasks on the task-spooler """
function tsp_set(nb_parallel::Int)
    run(`tsp -S 1`)
    for _ in 1:1:200
        try
            run(`tsp -k`)    
        catch _
            break
        end
    end
    run(`tsp -K`)
    run(`tsp /bin/true`)
    run(`tsp -S $nb_parallel`)
end

""" waits the last job to end."""
function tsp_wait()
    run(`tsp -w`)
end

end # module