mod m20260218_182252_convert_pks_to_serial_add_pid;

mod m20260218_000001_create_users;
mod m20260218_000002_create_projects;
mod m20260218_000003_create_testimonials;
mod m20260218_000004_create_tags;
mod m20260218_000005_create_forms;
mod m20260218_000006_create_widgets;
mod m20260218_000007_create_import_sources;
mod m20260218_000008_create_analytics_events;
mod m20260218_000009_create_api_keys;

rapina::migrations! {
    m20260218_000001_create_users,
    m20260218_000002_create_projects,
    m20260218_000003_create_testimonials,
    m20260218_000004_create_tags,
    m20260218_000005_create_forms,
    m20260218_000006_create_widgets,
    m20260218_000007_create_import_sources,
    m20260218_000008_create_analytics_events,
    m20260218_000009_create_api_keys,
    m20260218_182252_convert_pks_to_serial_add_pid,
}
