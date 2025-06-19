export interface HistoryEntry {
  name: string;
  href: string;
}

const PAGE_HISTORY: HistoryEntry[] = []

export function push_to_history(page: HistoryEntry) {
  PAGE_HISTORY.push(page);
  compact_history();
}

export function pop_history(): HistoryEntry {
  const popped = PAGE_HISTORY.pop();
  return popped ?? { name: "Home", href: "/" }
}

function compact_history() {
  for (let i = 1; i < PAGE_HISTORY.length; i++) {
    if (PAGE_HISTORY.at(i - 1)!.href === PAGE_HISTORY.at(i)!.href) {
      PAGE_HISTORY.splice(i - 1, 1);
    }
  }
}

export function get_previous_page(): HistoryEntry {
  if (PAGE_HISTORY.length < 2) {
    return {
      name: "Home",
      href: "/"
    };
  } else {
    return PAGE_HISTORY.at(-2)!;
  }
}

export function get_last_pages(n: number): HistoryEntry[] {
  return PAGE_HISTORY.slice(-n);
}

export function get_history_size(): number {
  return PAGE_HISTORY.length;
}
