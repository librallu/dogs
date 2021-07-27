module ParetoDiagram

using JSON
using Plots
using LaTeXStrings
using Statistics
using ColorSchemes
using Crayons
using Crayons.Box

"""
reads a JSON file describing the algorithm performance statistics.
"""
function read_performance_stats(filename::String)
    open(filename, "r") do f
        return JSON.parse(read(f,String))
    end
end

"""
given some arpd point lists, draw the curves
"""
function draw_arpd_diagrams(solvers_arpd_points, inst_class, output_dir)
    # create plot
    output_filename = "$(output_dir)/pareto_diagram_$(inst_class).pdf"
    p = plot(fontfamily="serif-roman")
    xlabel!("CPU time in seconds on instances of type `$(inst_class)'")
    ylabel!("average relative percentage deviation")
    for solver in keys(solvers_arpd_points)
        if "curve" in keys(solvers_arpd_points[solver])
            points = collect(zip(solvers_arpd_points[solver]["curve"]...))
            x = collect(points[1])
            y = collect(points[2])
            plot!(p, x, y, label=solver, linetype=:steppost)
        elseif "multiple_y" in keys(solvers_arpd_points[solver])
            tmp = solvers_arpd_points[solver]["multiple_y"]
            x = tmp[1]
            ys = tmp[2]
            scatter!(
                [x],[ys],
                label=solver,
                marker=:cross,
                interval=:confidence, level=0.95
            )
        else
        end
    end
    # export it
    gr()
    # pgfplotsx()
    savefig(p, output_filename)
end

"""
given a list of statistics for a specific algorithm on each instance of an
instance class, compute a set of points representing the evolution of the ARPD.
"""
function create_arpd_points(performances, references_primal, time_limit, nb_steps=100)
    step_size = float(time_limit)/nb_steps
    arpd_y = zeros(nb_steps+1) # one for each step
    step_where_all_have_solution = 1
    for (perf_id, perf) in enumerate(performances)
        previous_objective = 0.
        found_solution = false
        current_step = 1
        for point in perf["primal_pareto_diagram"]
            while point["time"] > current_step*step_size && current_step <= nb_steps
                arpd_y[current_step] += previous_objective
                if !found_solution
                    step_where_all_have_solution = max(
                        step_where_all_have_solution,
                        current_step
                    )
                end
                current_step += 1
            end
            found_solution = true
            previous_objective = (float(point["primal"])-references_primal[perf_id])/references_primal[perf_id]
        end
        while current_step <= nb_steps+1
            arpd_y[current_step] += previous_objective
            current_step += 1
        end
    end
    arpd_y = arpd_y .* (100. / length(performances))
    arpd_points = []
    for i in step_where_all_have_solution:nb_steps+1
        push!(arpd_points, (i*step_size, arpd_y[i]))
    end
    res = Dict()
    res["curve"] = arpd_points
    return res
end

"""
create pareto diagrams. They plot for each instance class, the evolution
of the solution quality (ARPD) relative to the time.
"""
function generate_pareto_diagrams(instances_csv, solver_variants, solver_variant_and_instance, output_dir)
    # build instance classes
    instance_classes = Dict() # instance class -> vector of instance data
    for inst in instances_csv
        if inst.class_name in keys(instance_classes)
            push!(instance_classes[inst.class_name], inst)
        else
            instance_classes[inst.class_name] = [inst]
        end
    end
    # for each instance class
    for inst_class in keys(instance_classes)
        references_primal = []
        for inst in instance_classes[inst_class]
            push!(references_primal, float(inst.bk_primal))
        end
        arpd_points = Dict()
        for s in solver_variants # for each solver
            performances = []
            solver_id = "$(s["name"])$(s["solver_params_compact"])"
            # if primal pareto diagram data exists
            time_limit = 0
            for inst in instance_classes[inst_class]
                time_limit = inst.time_limit
                inst_solver_id = "$(solver_id)_$(inst.name)"
                output_file = solver_variant_and_instance[inst_solver_id]["output_file_prefix"]*".stats.json"
                push!(performances, read_performance_stats(output_file))
            end
            if "primal_pareto_diagram" in keys(performances[1])
                arpd_points[solver_id] = create_arpd_points(performances, references_primal, time_limit)
            elseif "primal_list" in keys(performances[1])
                x = performances[1]["time_searched"]
                ys = collect(map((e)->e["primal_list"], performances))
                # FIXME add ARPD computation here
                ys = collect(zip(ys...))
                ys = collect(map((e)->mean(e), ys))
                res = Dict()
                res["multiple_y"] = [x, ys]
                arpd_points[solver_id] = res
            else
                println(RED_FG("'best_primal' nor 'primal_list' is found"))
            end
        end
        draw_arpd_diagrams(arpd_points, inst_class, output_dir)
    end
end

end # module