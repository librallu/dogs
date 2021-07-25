module TableOfResults

"""
creates a table of results (best solution for each algorithm)
"""
function generate_table_of_results(instances_csv, solver_variants, solver_variant_and_instance, output_filename)
    res = "instance,"
    for s in solver_variants
        res *= s["name"]*s["solver_params_compact"] * ","
    end
    res *= "\n"
    for inst in instances_csv
        res *= inst.name
        res *= "\n"
    end
    println(res)
end

end # module