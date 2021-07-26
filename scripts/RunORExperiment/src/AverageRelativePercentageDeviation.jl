module AverageRelativePercentageDeviation

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
creates an average-relative-percentage-deviation table. One row per instance class
"""
function generate_arpd_table(instances_csv, solver_variants, solver_variant_and_instance, output_filename)
    res = "instance_class"
    for s in solver_variants
        res *= ","*s["name"]*s["solver_params_compact"]
    end
    res *= "\n"
    # build instance classes
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
        for s in solver_variants # for each solver
            arpd = 0
            for inst in instance_classes[inst_class]
                inst_solver_id = "$(s["name"])$(s["solver_params_compact"])_$(inst.name)"
                output_file = solver_variant_and_instance[inst_solver_id]["output_file_prefix"]*".stats.json"
                stats = read_performance_stats(output_file)
                reference_primal = float(inst.bk_primal)
                arpd += (float(stats["best_primal"])-reference_primal)/reference_primal
            end
            arpd = (arpd*100.)/float(length(instance_classes[inst_class]))
            res *= ",$(arpd)"
        end
        res *= "\n"
    end
    f = open(output_filename, "w")
    write(f, res)
    close(f)
end

end # module