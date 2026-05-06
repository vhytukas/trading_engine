const TradesPanel = ({ trades }) => {
  return (
    <section className="panel trades">
      <h2>Trades</h2>
      <table>
        <thead>
          <tr>
            <th>Time</th>
            <th>Side</th>
            <th>Price</th>
            <th>Qty</th>
          </tr>
        </thead>
        <tbody>
          {trades.map((trade, index) => (
            <tr key={`${trade.time}-${index}`}>
              <td>{trade.time}</td>
              <td className={trade.side === "BUY" ? "bid" : "ask"}>{trade.side}</td>
              <td>{trade.price}</td>
              <td>{trade.qty}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
};

export default TradesPanel;
