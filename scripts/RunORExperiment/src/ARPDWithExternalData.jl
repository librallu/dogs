module ARPDWithExternalData

using JSON
using Crayons
using Crayons.Box
using Statistics


function get_best_value_before_time(stats, time_limit, epsilon=0.01)
    if time_limit < epsilon
        time_limit = epsilon
    end
    if "primal_list" in keys(stats)
        return sum(stats["primal_list"])/length(stats["primal_list"])
    end
    previous = nothing
    for point in stats["primal_pareto_diagram"]
        if point["time"] > time_limit
            return previous
        end
        previous = point["primal"]
    end
    return previous
end

"""
creates an average-relative-percentage-deviation table.
One row per instance class.
Generates a CSV file and a latex file.
:arpd_refs: instance name -> objective value
"""
function generate_external_arpd_table(instances_csv, arpd_refs, solver_variants, solver_variant_and_instance, output_filename, external_name, external_data, variants_list)
    filtered_solver_variants = solver_variants
    if variants_list !== nothing
        filtered_solver_variants = []
        for e in variants_list
            for s in solver_variants
                id = s["name"]*s["solver_params_compact"]
                if id == e
                    push!(filtered_solver_variants, s)
                end
            end
        end
    end
    # tex preembule
    res_tex = "\\begin{tabular}{c|"
    res_tex *= "c|"
    for _ in filtered_solver_variants
        res_tex *= "c"
    end
    res_tex *= "}\n"
    res_tex *= "instance class & $(external_name)"
    for s in filtered_solver_variants # for each solver
        solver_name = "$(s["name"])$(s["solver_params_compact"])"
        solver_name = replace(solver_name, "_"=>"\\_")
        res_tex *= " & $(solver_name)"
    end
    res_tex *= " \\\\ \n\\hline\n"
    # csv preembule
    res = "instance_class"
    for s in filtered_solver_variants
        res *= ","*s["name"]*s["solver_params_compact"]
    end
    res *= "\n"
    # build the sorted instance classes
    instance_classes_sorted = []
    instance_classes = Dict() # instance class -> vector of instance data
    for inst in instances_csv
        if inst.class_name in keys(instance_classes)
            push!(instance_classes[inst.class_name], inst)
        else
            push!(instance_classes_sorted, inst.class_name)
            instance_classes[inst.class_name] = [inst]
        end
    end
    # for each instance class
    for inst_class in instance_classes_sorted
        res *= inst_class
        solver_lists = Dict()
        solver_arpd = Dict()
        # display external
        external_time = external_data[inst_class]["time"]
        external_arpd = external_data[inst_class]["arpd"]
        res *= ",$(external_arpd)"
        # display other solvers
        for s in filtered_solver_variants # for each solver
            arpd = 0
            solver_name = "$(s["name"])$(s["solver_params_compact"])"
            solver_lists[solver_name] = []
            for inst in instance_classes[inst_class]
                inst_solver_id = "$(solver_name)_$(inst.name)"
                stats = solver_variant_and_instance[inst_solver_id]["stats"]
                reference_primal = arpd_refs[inst.name]
                solver_primal = get_best_value_before_time(stats, external_time)
                push!(solver_lists[solver_name], solver_primal)
                arpd += (solver_primal-reference_primal)/reference_primal
            end
            arpd = (arpd*100.)/float(length(instance_classes[inst_class]))
            res *= ",$(arpd)"
            solver_arpd[solver_name] = arpd
        end
        # update CSV
        res *= "\n"
        # update tex
        pad = 12
        res_tex *= "$(rpad(inst_class, pad, " "))"
        tmp_arpd = round(external_arpd, digits=2)
        res_tex *= " & $(rpad(tmp_arpd, pad, " "))"
        for s in filtered_solver_variants
            s_name = "$(s["name"])$(s["solver_params_compact"])"
            arpd = round(solver_arpd[s_name], digits=2)
            v_tex = "$(arpd)"
            res_tex *= " & $(rpad(v_tex, pad, " "))"
        end
        res_tex *= " \\\\ \n"
    end
    # write CSV
    f = open(output_filename, "w")
    write(f, res)
    close(f)
    # write tex
    res_tex *= "\\end{tabular}"
    f = open(output_filename*".tex", "w")
    write(f, res_tex)
    close(f)
end

end # module