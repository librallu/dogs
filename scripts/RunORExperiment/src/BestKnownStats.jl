module BestKnownStats

function generate_best_known_stats(instances_csv, solver_variants, solver_variant_and_instance, output_filename)
    res = "solver,nb_instances,nb_better_bk,nb_strictly_better_bk,nb_proved_optimal\n"
    for s in solver_variants
        nb_better_bk = 0
        nb_strictly_better_bk = 0
        nb_proved_optimal = 0
        nb_instances = 0
        for inst in instances_csv
            nb_instances += 1
            bk_primal = inst.bk_primal
            inst_solver_id = "$(s["name"])$(s["solver_params_compact"])_$(inst.name)"
            stats = solver_variant_and_instance[inst_solver_id]["stats"]
            best_primal = nothing
            is_optimal = false
            if "best_primal" in keys(stats)
                best_primal = stats["best_primal"]
                is_optimal = stats["is_optimal"]
            elseif "primal_list" in keys(stats)
                best_primal = min(stats["primal_list"]...)
            else
                println(RED_FG("'best_primal' nor 'primal_list' is found"))
                res *= ",?,?,?"
            end
            # update stats
            if is_optimal
                nb_proved_optimal += 1
            end
            if best_primal < bk_primal
                nb_strictly_better_bk += 1
            end
            if best_primal <= bk_primal
                nb_better_bk += 1
            end
        end
        res *= "$(s["id"]),$(nb_instances),$(nb_better_bk),$(nb_strictly_better_bk),$(nb_proved_optimal)\n"
    end
    f = open(output_filename, "w")
    write(f, res)
    close(f)
end

end # module