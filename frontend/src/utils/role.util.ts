import type { TFunction } from "i18next";

const builtinRoles = new Map([
	["Member", "roles.member"],
	["Any", "roles.any"],
]);

// eslint-disable-next-line id-length
export function roleLabel(t: TFunction, name: string): string {
	const key = builtinRoles.get(name);
	return key ? t(key) : name;
}
