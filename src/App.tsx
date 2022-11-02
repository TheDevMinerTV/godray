import { MdSearch } from "react-icons/md";

function App() {
  return (
    <div className="yeet">
      <div className="flex items-center select-none">
        <MdSearch className="text-2xl text-icon select-none" />

        <input
          type="text"
          placeholder="Godray Search"
          className="ml-2 bg-transparent text-xl w-full text-input select-text"
          autoFocus
        />
      </div>
    </div>
  );
}

export default App;
