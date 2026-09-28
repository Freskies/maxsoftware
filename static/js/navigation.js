// NAVBAR

const navigation = document.querySelector(".floating-navigation");

if (navigation) {
	const items = [...navigation.querySelectorAll(".navigation-item")];
	navigation.dataset.itemCount = String(items.length);
	navigation.style.setProperty("--navigation-items", items.length);

	function normalizePath (pathname) {
		return pathname.replace(/\/+$/, "") || "/";
	}

	function updateActiveNavigation () {
		const currentPath = normalizePath(window.location.pathname);

		const activeIndex = items.findIndex(item => {
			const itemPath = new URL(item.href).pathname;
			return normalizePath(itemPath) === currentPath;
		});

		items.forEach((item, index) => {
			if (index === activeIndex) item.setAttribute("aria-current", "page");
			else item.removeAttribute("aria-current");
		});

		navigation.toggleAttribute("data-has-active", activeIndex !== -1);

		if (activeIndex !== -1) navigation.style.setProperty("--active-index", activeIndex);
	}

	document.addEventListener("htmx:after:history:update", updateActiveNavigation);
	window.addEventListener("popstate", updateActiveNavigation);
	updateActiveNavigation();
}

// LINK IN-PAGE (SCROLL TO)

document.addEventListener("click", event => {
	if (!(event.target instanceof Element)) return;

	const link = event.target.closest("a[data-scroll-to]");
	if (!link) return;

	if (event.button !== 0 || event.ctrlKey || event.metaKey || event.shiftKey || event.altKey) return;

	const href = link.getAttribute("href");
	if (!href?.startsWith("#")) return;

	const target = document.getElementById(href.slice(1));
	if (!target) return;

	event.preventDefault();
	target.scrollIntoView({ block: "start" });
});