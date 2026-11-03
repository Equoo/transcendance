import type { TFunction } from "i18next";

// Roles created by the backend itself (Roles.DefaultRole and EventRole.Implicit):
// their names are fixed, so they are shown translated instead of as stored.
const builtinRoles = new Map([
	["Member", "roles.member"],
	["Any", "roles.any"],
]);

// eslint-disable-next-line id-length
export function roleLabel(t: TFunction, name: string): string {
	const key = builtinRoles.get(name);
	return key ? t(key) : name;
}
