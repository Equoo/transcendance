/* eslint-disable @typescript-eslint/no-unsafe-assignment */
import type { Locale } from "date-fns";
import { enUS, fr as frLocale, zhTW } from "date-fns/locale";
import i18n from "i18next";
import { initReactI18next } from "react-i18next";

import en from "./en/translation";
import fr from "./fr/translation";
import tc from "./tc/translation";

const resources = {
	...en,
	...fr,
	...tc,
};

const supportedLanguages = ["fr", "en", "tc"] as const;

export const languages = [
	{ code: "fr", flag: "🇫🇷", label: "Français" },
	{ code: "en", flag: "🇬🇧", label: "English" },
	{ code: "tc", flag: "🇹🇼", label: "繁體中文" },
] as const;

const storageKey = "language";

function readStoredLanguage(): string | null {
	try {
		return localStorage.getItem(storageKey);
	} catch {
		return null;
	}
}

function storeLanguage(lng: string): void {
	localStorage.setItem(storageKey, lng);
}

function detectLanguage(): string {
	if (typeof window === "undefined") {
		return "en";
	}

	const stored = readStoredLanguage();
	if (stored !== null && supportedLanguages.some((lng) => lng === stored)) {
		return stored;
	}

	for (const browserLanguage of navigator.languages) {
		const [prefix] = browserLanguage.toLowerCase().split("-");
		if (prefix === "fr") {
			return "fr";
		}
		if (prefix === "en") {
			return "en";
		}
		if (prefix === "zh") {
			return "tc";
		}
	}
	return "en";
}

// The SPA shell is prerendered in English: start in English so the first
// client render matches it, then switch with applyDetectedLanguage().
void i18n.use(initReactI18next).init({
	resources,

	lng: "en",
	fallbackLng: "en",
	initAsync: false,

	interpolation: {
		escapeValue: false,
	},
});

export function applyDetectedLanguage(): void {
	const lng = detectLanguage();
	if (lng !== i18n.language) {
		void i18n.changeLanguage(lng);
	}
}

const htmlLang: Record<string, string> = { en: "en", fr: "fr", tc: "zh-TW" };

const dateFnsLocales: Record<string, Locale> = {
	en: enUS,
	fr: frLocale,
	tc: zhTW,
};

export function dateLocale(lng: string): string {
	return htmlLang[lng] ?? "en";
}

export function dateFnsLocale(lng: string): Locale {
	return dateFnsLocales[lng] ?? enUS;
}

i18n.on("languageChanged", (lng) => {
	if (typeof window !== "undefined") {
		storeLanguage(lng);
	}
});

export default i18n;
