import { useEffect } from "react";

export function useViewportHeight(): void {
	useEffect(() => {
		const viewport = window.visualViewport;
		const root = document.documentElement;

		function update(): void {
			if (!viewport) {
				return;
			}
			root.style.setProperty("--app-height", `${viewport.height}px`);
			window.scrollTo(0, 0);
		}

		update();
		viewport?.addEventListener("resize", update);
		return (): void => {
			viewport?.removeEventListener("resize", update);
			root.style.removeProperty("--app-height");
		};
	}, []);
}
