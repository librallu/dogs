module BestPrimalTable

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
given a statistics object and a primal_objective, returns the time to find it, or "-"
"""
function time_to_objective(stats, objective)
    res = "-"
    for point in stats["primal_pareto_diagram"]
        if point["primal"] <= objective
            res = "$(point["time"])"
            break
        end
    end
    return res
end

"""
given a statistics object, returns the time to optimality or "-" if not optimal
"""
function time_to_optimal(stats)
    if stats["is_optimal"]
        return "$(stats["time_searched"])"
    else
        return "-"
    end
end

"""
creates a "best-primal-bound" table (best solution found for each algorithm)
"""
function generate_best_primal_table(instances_csv, solver_variants, solver_variant_and_instance, output_filename)
    res = "instance,best_known"
    for s in solver_variants
        res *= ",$(s["name"])$(s["solver_params_compact"])_primal"
        res *= ",$(s["name"])$(s["solver_params_compact"])_time_to_best_known"
        res *= ",$(s["name"])$(s["solver_params_compact"])_time_opt"
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
            res *= ","*"$(time_to_objective(stats, inst.bk_primal))"
            res *= ","*"$(time_to_optimal(stats))"
        end
        res *= "\n"
    end
    f = open(output_filename, "w")
    write(f, res)
    close(f)
end

end # module