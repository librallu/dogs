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
given a statistics object and a primal_objective, returns the time to find it, or "-"
"""
function time_to_improve(stats, objective)
    res = "-"
    for point in stats["primal_pareto_diagram"]
        if point["primal"] < objective
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
1234567 -> 1.234.567
"""
function human_format_number(n)
    res = ""
    while n > 1000
        res = ".$(lpad("$(n % 1000)",3,"0"))$(res)"
        n = n ÷ 1000
    end
    res = "$(n)$(res)"
    res
end

"""
adds bold to the latex string
"""
function latex_bold(s)
    "{\\bf $(s)}"
end

"""
creates a "best-primal-bound" table (best solution found for each algorithm)
"""
function generate_best_primal_table(instances_csv, custom_external, solver_variants, solver_variant_and_instance, output_filename, time_to_best_known=true, time_opt=true)
    res_tex = "\\begin{tabular}{cc|"
    for _ in custom_external 
        res_tex *= "c"
    end
    res_tex *= "|"
    for _ in solver_variants
        res_tex *= "c"
    end
    res_tex *="}\n"
    res_tex *= "instance & best-known"
    # add external data
    external_data_list = collect(keys(custom_external))
    for s in external_data_list
        res_tex *= " & $(replace(s,"_"=>"\\_"))"
    end
    for s in solver_variants
        res_tex *= replace(" & $(s["name"])$(s["solver_params_compact"])","_"=>"\\_")
    end
    res_tex *= " \\\\ \n \\hline \n"
    res = "instance,best_known"
    for k in external_data_list
        res *= ",$(k)"
    end
    # add solver variants
    for s in solver_variants
        res *= ",$(s["name"])$(s["solver_params_compact"])_primal"
        if time_to_best_known
            res *= ",$(s["name"])$(s["solver_params_compact"])_time_to_best_known"
            res *= ",$(s["name"])$(s["solver_params_compact"])_time_to_improve"
        end
        if time_opt
            res *= ",$(s["name"])$(s["solver_params_compact"])_time_opt"
        end
    end
    res *= "\n"
    pad = 16
    for inst in instances_csv
        res *= inst.name*","*"$(inst.bk_primal)"
        res_tex *= replace("$(rpad(inst.name, pad, " ")) & $(rpad(human_format_number(inst.bk_primal), pad, " "))","_"=>"\\_")
        tex_vals = []
        # add external data
        for k in external_data_list
            v = custom_external[k]["results"][inst.name]
            res *= ",$(v)"
            push!(tex_vals, v)
        end
        # add solver variants
        for s in solver_variants
            inst_solver_id = "$(s["name"])$(s["solver_params_compact"])_$(inst.name)"
            stats = solver_variant_and_instance[inst_solver_id]["stats"]
            if "best_primal" in keys(stats)
                v = stats["best_primal"]
                res *= ","*"$(v)"
                push!(tex_vals, v)
                if time_to_best_known
                    res *= ","*"$(time_to_objective(stats, inst.bk_primal))"
                    res *= ","*"$(time_to_improve(stats, inst.bk_primal))"
                end
                if time_opt
                    res *= ","*"$(time_to_optimal(stats))"
                end
            elseif "primal_list" in keys(stats)
                v = min(stats["primal_list"]...)
                res *= ",$(v)"
                push!(tex_vals, v)
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
        # write latex line
        min_val = min(filter((v)-> v!== missing, tex_vals)...)
        for v in tex_vals
            if v !== missing
                # println("min_val:$(min_val)\tbkprimal:$(inst.bk_primal)")
                tex_str = human_format_number(v)
                if v == min_val && v <= inst.bk_primal
                    tex_str = latex_bold(tex_str)
                end
                res_tex *= " & $(rpad(
                    "$(tex_str)", pad, " "
                ))"
            else
                res_tex *= " & $(rpad(
                    "-", pad, " "
                ))"
            end
        end
        res *= "\n"
        res_tex *= "\\\\ \n"
    end
    f = open(output_filename, "w")
    write(f, res)
    close(f)
    # write tex file
    res_tex *= "\\end{tabular}"
    f = open(output_filename*".tex", "w")
    write(f, res_tex)
    close(f)
end

end # module