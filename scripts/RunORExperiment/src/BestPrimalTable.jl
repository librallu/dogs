module BestPrimalTable

using JSON
using Crayons
using Crayons.Box

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
function generate_best_primal_table(instances_csv, custom_external, solver_variants, solver_variant_and_instance, output_filename, time_to_best_known=true, time_opt=true)
    res = "instance,best_known"
    # add external data
    external_data_list = collect(keys(custom_external))
    for k in external_data_list
        res *= ",$(k)"
    end
    # add solver variants
    for s in solver_variants
        res *= ",$(s["name"])$(s["solver_params_compact"])_primal"
        if time_to_best_known
            res *= ",$(s["name"])$(s["solver_params_compact"])_time_to_best_known"
        end
        if time_opt
            res *= ",$(s["name"])$(s["solver_params_compact"])_time_opt"
        end
    end
    res *= "\n"
    for inst in instances_csv
        res *= inst.name*","*"$(inst.bk_primal)"
        # add external data
        for k in external_data_list
            res *= ",$(custom_external[k]["results"][inst.name])"
        end
        # add solver variants
        for s in solver_variants
            inst_solver_id = "$(s["name"])$(s["solver_params_compact"])_$(inst.name)"
            stats = solver_variant_and_instance[inst_solver_id]["stats"]
            if "best_primal" in keys(stats)
                res *= ","*"$(stats["best_primal"])"
                if time_to_best_known
                    res *= ","*"$(time_to_objective(stats, inst.bk_primal))"
                end
                if time_opt
                    res *= ","*"$(time_to_optimal(stats))"
                end
            elseif "primal_list" in keys(stats)
                res *= ",$(min(stats["primal_list"]...))"
                if time_to_best_known
                    res *= ",-"
                end
                if time_opt
                    res *= ",-"
                end
            else
                println(RED_FG("'best_primal' nor 'primal_list' is found"))
                res *= ",?"
                if time_to_best_known
                    res *= ",?"
                end
                if time_opt
                    res *= ",?"
                end
            end
        end
        res *= "\n"
    end
    f = open(output_filename, "w")
    write(f, res)
    close(f)
end

end # module