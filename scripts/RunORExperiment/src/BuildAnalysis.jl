"""
Builds various analysis for the experiment.
"""
module BuildAnalysis

using CSV

include("PerformExperiment.jl")

include("BestKnownStats.jl")
include("BestPrimalTable.jl")
include("AverageRelativePercentageDeviation.jl")
include("ParetoDiagram.jl")


function build_analysis(configuration, common, instances_csv, solver_variants, solver_variant_with_instance)
    # create ARPD references
    configuration_filename = configuration["filename"]
    arpd_refs = Dict() # instance name -> reference value
    if "analysis" in keys(configuration) && "arpd_ref" in keys(configuration["analysis"])
        # if reference is given, read it 
        arpd_ref_csv = PerformExperiment.read_csv(abspath(joinpath(
            configuration_filename, "..",
            configuration["analysis"]["arpd_ref"])
        ))
        for inst in arpd_ref_csv
            arpd_refs[inst.name] = inst.reference_objective
        end
    else
        # otherwise set the best_known as reference
        for inst in instances_csv
            arpd_refs[inst.name] = inst.bk_primal
        end
    end
    # possibly add external results ARPD
    external_arpds_data = Dict()
    if "analysis" in keys(configuration) && "external_arpd_results" in keys(configuration["analysis"])
        custom_arpds = configuration["analysis"]["external_arpd_results"]
        for external_algo in custom_arpds
            name = external_algo["name"]
            time_col = external_algo["time"]
            arpd_col = external_algo["arpd"]
            cpu_regularization_factor = 1.
            if "cpu_regularization_factor" in keys(external_algo)
                cpu_regularization_factor = external_algo["cpu_regularization_factor"]
            end
            csv_filename = abspath(joinpath(
                configuration_filename, "..",
                external_algo["file"]
            ))
            csv = PerformExperiment.read_csv(csv_filename)
            contents = Dict()
            for line in csv
                contents[line.instance_class] = Dict()
                contents[line.instance_class]["time"] = getindex(
                    line, Meta.parse("$(time_col)")
                ) * cpu_regularization_factor
                contents[line.instance_class]["arpd"] = getindex(
                    line, Meta.parse("$(arpd_col)")
                )
            end
            external_arpds_data[name] = contents
        end
    end
    # possibly add some external best known solutions
    external_best_known = Dict()
    if "analysis" in keys(configuration) && "external_best_known_results" in keys(configuration["analysis"])
        external_best_known_results = configuration["analysis"]["external_best_known_results"]
        for external_bk in external_best_known_results
            name = external_bk["name"]
            csv_filename = abspath(joinpath(
                configuration_filename, "..",
                external_bk["file"]
            ))
            csv = PerformExperiment.read_csv(csv_filename)
            contents = Dict()
            contents["name"] = name
            contents["solver_params_compact"] = ""
            contents["results"] = Dict()
            for line in csv
                contents["results"][line.name] = getindex(
                    line, Meta.parse("$(external_bk["column"])")
                )
            end
            external_best_known[name] = contents
        end
    end
    # generate best_known stats
    println("best knonwn stats")
    BestKnownStats.generate_best_known_stats(
        instances_csv,
        solver_variants,
        solver_variant_with_instance,
        "$(common["output_directory"])/analysis/best_known_stats.csv"
    )
    # generate best primal table
    println("best primal table generation")
    BestPrimalTable.generate_best_primal_table(
        instances_csv,
        external_best_known,
        solver_variants,
        solver_variant_with_instance,
        "$(common["output_directory"])/analysis/best_primal_bounds.csv"
    )
    # generate ARPD table
    println("ARPD table generation")
    AverageRelativePercentageDeviation.generate_arpd_table(
        instances_csv,
        arpd_refs,
        solver_variants,
        solver_variant_with_instance,
        "$(common["output_directory"])/analysis/arpd_table.csv"
    )
    # generate Pareto diagrams
    println("pareto diagram generation")
    pareto_diagram_path = "$(common["output_directory"])/analysis/pareto_diagrams/"
    mkpath(pareto_diagram_path)
    instance_classes = PerformExperiment.build_instance_classes(instances_csv)
    ParetoDiagram.generate_pareto_diagrams(
        instance_classes,
        arpd_refs,
        external_arpds_data,
        solver_variants,
        solver_variant_with_instance,
        pareto_diagram_path
    )
    # generate custom pareto diagrams
    if "custom_pareto_diagrams" in keys(configuration["analysis"])
        println("creating custom pareto diagrams")
        diagrams_path = common["output_directory"]*"/analysis/custom_pareto/"
        mkpath(diagrams_path)
        custom_pareto_diagrams = configuration["analysis"]["custom_pareto_diagrams"]
        for diagram in custom_pareto_diagrams
            algo_set = Set(diagram["algorithms"])
            class_set = diagram["classes"]
            # build custom instance classes
            custom_instance_classes = Dict()
            for inst_class in class_set
                custom_instance_classes[inst_class] = instance_classes[inst_class]
            end
            # build solver variants
            custom_solver_variants = []
            for solver in solver_variants
                if solver["id"] in algo_set
                    push!(custom_solver_variants, solver)
                end
            end
            # build external arpd data
            custom_external_arpds_data = Dict()
            for d in keys(external_arpds_data)
                if d in algo_set
                    custom_external_arpds_data[d] = external_arpds_data[d]
                end
            end
            # call the diagram construction
            ParetoDiagram.generate_pareto_diagrams(
                custom_instance_classes,
                arpd_refs,
                custom_external_arpds_data,
                custom_solver_variants,
                solver_variant_with_instance,
                diagrams_path,
                diagram["name"]
            )
        end
    end
    # generate custom best known
    if "custom_best_known_tables" in keys(configuration["analysis"])
        println("creating custom best-known tables")
        bk_path = common["output_directory"]*"/analysis/custom_best_known_tables/"
        mkpath(bk_path)
        custom_bk = configuration["analysis"]["custom_best_known_tables"]
        for bk in custom_bk
            algo_set = Set(bk["algorithms"])
            name = bk["name"]
            # build solver variants
            custom_solver_variants = []
            for solver in solver_variants
                if solver["id"] in algo_set
                    push!(custom_solver_variants, solver)
                end
            end
            # add external information
            custom_external = Dict() # name -> inst_name -> value
            for s in keys(external_best_known)
                if s in algo_set
                    custom_external[s] = external_best_known[s]
                end
            end
            # build solver_variant_with_instance
            BestPrimalTable.generate_best_primal_table(
                instances_csv,
                custom_external,
                custom_solver_variants,
                solver_variant_with_instance,
                "$(bk_path)/$(name).csv",
                bk["time_to_best_known"],
                bk["time_to_opt"]
            )
        end
    end
end


end # module