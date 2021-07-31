module AverageRelativePercentageDeviation

using JSON
using HypothesisTests
using Crayons
using Crayons.Box
using Statistics

"""
returns true if sequence A is statistically significantly better than sequence B.
According to the signed Wilcoxon signed-rank test
"""
function compare_sequences_wilcoxon(a,b,plimit=0.05)
    diff = a .- b
    pv = pvalue(SignedRankTest(diff))
    pv <= plimit && sum(diff) < 0
end


"""
creates an average-relative-percentage-deviation table.
One row per instance class.
Generates a CSV file and a latex file.
:arpd_refs: instance name -> objective value
"""
function generate_arpd_table(instances_csv, arpd_refs, solver_variants, solver_variant_and_instance, output_filename)
    # tex preembule
    res_tex = "\\begin{tabular}{c|"
    for _ in solver_variants
        res_tex *= "c"
    end
    res_tex *= "}\n"
    res_tex *= "instance class"
    for s in solver_variants # for each solver
        solver_name = "$(s["name"])$(s["solver_params_compact"])"
        solver_name = replace(solver_name, "_"=>"\\_")
        res_tex *= " & $(solver_name)"
    end
    res_tex *= " \\\\ \n\\hline\n"
    # csv preembule
    res = "instance_class"
    for s in solver_variants
        res *= ","*s["name"]*s["solver_params_compact"]
    end
    res *= ",wilcoxon_best"
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
        for s in solver_variants # for each solver
            arpd = 0
            solver_name = "$(s["name"])$(s["solver_params_compact"])"
            solver_lists[solver_name] = []
            for inst in instance_classes[inst_class]
                inst_solver_id = "$(solver_name)_$(inst.name)"
                stats = solver_variant_and_instance[inst_solver_id]["stats"]
                reference_primal = arpd_refs[inst.name]
                if "best_primal" in keys(stats)
                    solver_primal = float(stats["best_primal"])
                elseif "primal_list" in keys(stats)
                    solver_primal = mean(stats["primal_list"])
                else
                    println(RED_FG("'best_primal' nor 'primal_list' is found"))
                    solver_primal = 0.
                end
                push!(solver_lists[solver_name], solver_primal)
                arpd += (solver_primal-reference_primal)/reference_primal
            end
            arpd = (arpd*100.)/float(length(instance_classes[inst_class]))
            res *= ",$(arpd)"
            solver_arpd[solver_name] = arpd
        end
        best_wilcoxon = "-"
        for s1 in keys(solver_lists)
            is_better = true
            for s2 in keys(solver_lists)
                if s1 != s2
                    is_better = compare_sequences_wilcoxon(
                        solver_lists[s1],
                        solver_lists[s2]
                    )
                    if !is_better
                        break
                    end
                end
            end
            if is_better
                best_wilcoxon = s1
                break
            end
        end
        # update CSV
        res *= ",$(best_wilcoxon)"
        res *= "\n"
        # update tex
        res_tex *= "$(inst_class)"
        for s in solver_variants
            s_name = "$(s["name"])$(s["solver_params_compact"])"
            arpd = round(solver_arpd[s_name], digits=2)
            if best_wilcoxon == s_name
                res_tex *= " & \\textbf{$(arpd)}"
            else
                res_tex *= " & $(arpd)"
            end
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