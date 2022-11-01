import { MdSearch } from "react-icons/md";

function App() {
  return (
    <div className="yeet">
      <div className="flex items-center">
        <MdSearch className="text-2xl text-icon" />

        <input
          type="text"
          placeholder="Godray Search"
          className="ml-2 bg-transparent text-xl w-full text-input"
          autoFocus
        />
      </div>
    </div>
  );
}

export default App;
