#include <stdlib.h>
#include "pk_rs.h"

// TODO: better/proper error handling.
PkrsPackage **pkrs_get_package_details(const char *package_name)
{
	GError *error = NULL;
	PkError *error_code = NULL;
	PkResults *results = NULL;
	GPtrArray *array = NULL;
	PkPackage *item = NULL;
	gchar **values = NULL;
	size_t i;
	PkrsPackage **packages = NULL;

	PkTask *task = pk_task_new();

	// Resolve package name.
	values = g_new0(gchar *, 1 + 1);
	values[0] = g_strdup(package_name);
	values[1] = NULL;
	results = pk_task_resolve_sync(task, PK_FILTER_ENUM_NONE, values, NULL, NULL, NULL, &error);

	// Check error code.
	error_code = pk_results_get_error_code(results);
	if (error_code != NULL)
	{
		return packages;
	}

	// Retrieve details of package(s).
	array = pk_results_get_package_array(results);
	packages = g_new0(PkrsPackage *, array->len + 1);
	for (i = 0; i < array->len; i++)
	{
		item = g_ptr_array_index(array, i);
		PkrsPackage package = {
			.id = g_strdup(pk_package_get_id(item)),
			.name = g_strdup(pk_package_get_name(item)),
			.version = g_strdup(pk_package_get_version(item)),
			.summary = g_strdup(pk_package_get_summary(item)),
			.arch = g_strdup(pk_package_get_arch(item)),
			.data = g_strdup(pk_package_get_data(item)),
		};
		packages[i] = &package;
	}

	// Cleanup and return.
	g_strfreev(values);
	g_object_unref(task);
	if (error_code != NULL)
	{
		g_object_unref(error_code);
	}
	if (array != NULL)
	{
		g_ptr_array_unref(array);
	}
	if (results != NULL)
	{
		g_object_unref(results);
	}

	return packages;
}
