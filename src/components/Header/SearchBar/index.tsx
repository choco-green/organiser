import { FaSearch } from 'react-icons/fa';

export default function SearchBar() {
  /**
   * TODO: Once clicked, the search bar should display a modal with a search
   * input and a list of results, the expected behaviour is TBD, but expected
   * results to include events, widgets, pages, etc.
   */

  return (
    <button type="button" className="flex h-8 w-96 items-center rounded border border-ctp-overlay0 bg-ctp-surface0 px-4 py-1">
      <FaSearch className="mr-3 fill-ctp-overlay0" />
      <div className="text-ctp-overlay0">Search</div>
    </button>
  );
}
