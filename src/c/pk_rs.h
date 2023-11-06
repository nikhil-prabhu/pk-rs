#include <PackageKit/packagekit-glib2/packagekit.h>

/**
 * Represents a package and its details.
 */
typedef struct PkrsPackage
{
	const gchar *id;
	const gchar *name;
	const gchar *version;
	const gchar *summary;
	const gchar *arch;
	const gchar *data;
} PkrsPackage;

/**
 * Retrieves package details using the specified package name.
 */
PkrsPackage **pkrs_get_package_details(const char *package_name);
