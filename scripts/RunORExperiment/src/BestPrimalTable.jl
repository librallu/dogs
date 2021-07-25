module TableOfResults

using JSON

"""
reads a JSON file describing the algorithm performance statistics.
"""
function read_performance_stats(filename::String)
    open(filename, "r") do f
        return JSON.parse(read(f,String))
    end
end


"""
creates a table of results (best solution for each algorithm)
"""
function generate_best_primal_table(instances_csv, solver_variants, solver_variant_and_instance, output_filename)
    res = "instance,best_known"
    for s in solver_variants
        res *= ","*s["name"]*s["solver_params_compact"]
    end
    res *= "\n"
    for inst in instances_csv
        res *= inst.name*","*"$(inst.bk_primal)"
        for s in solver_variants
            inst_solver_id = "$(s["name"])$(s["solver_params_compact"])_$(inst.name)"
            output_file = solver_variant_and_instance[inst_solver_id]["output_file_prefix"]*".stats.json"
            stats = read_performance_stats(output_file)
            # println(stats)
            res *= ","*"$(stats["best_primal"])"
        end
        res *= "\n"
    end
    f = open(output_filename, "w")
    write(f, res)
    close(f)
end

end # module