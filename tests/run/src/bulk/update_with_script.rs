bulk_create(json!({})).id(1);
bulk_update_script("ctx._source.title = params.newTitle")
    .script_fluent(|script| script.param("newTitle", "Updated"))
    .id(1);