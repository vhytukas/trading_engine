const TopBar = ({ statusText }) => {
  return (
    <header className="topbar">
      <div>
        <p className="symbol">BTC / USDT</p>
        <h1>Orderbook</h1>
      </div>
      <span className="status">{statusText}</span>
    </header>
  );
};

export default TopBar;
