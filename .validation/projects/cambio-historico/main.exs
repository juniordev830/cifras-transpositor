defmodule HistoricalRates do
  def parse(text) do
    [header | rows] = String.split(text, "\n", trim: true)
    if String.trim_trailing(header, "\r") != "data\tmoeda\tmicros", do: raise(ArgumentError, "Cabeçalho TSV inválido")
    parsed = Enum.map(rows, fn row ->
      [date, currency, value] = String.split(String.trim_trailing(row,"\r"), "\t")
      {:ok, day} = Date.from_iso8601(date)
      {micros, ""} = Integer.parse(value)
      if not Regex.match?(~r/^[A-Z]{3}$/, currency) or micros <= 0, do: raise(ArgumentError, "Cotação inválida")
      %{date: day, currency: currency, micros: micros}
    end)
    keys = Enum.map(parsed, &{&1.date, &1.currency})
    if length(Enum.uniq(keys)) != length(keys), do: raise(ArgumentError, "Data/moeda duplicada")
    parsed
  end
  def compare(rows, currency) do
    selected = rows |> Enum.filter(&(&1.currency == currency)) |> Enum.sort_by(&Date.to_gregorian_days(&1.date))
    if length(selected) < 2, do: raise(ArgumentError, "São necessárias duas datas da mesma moeda")
    first = hd(selected); last = List.last(selected)
    %{first: first, last: last, variation_basis_points: div((last.micros - first.micros) * 10_000, first.micros)}
  end
end
if System.argv() == ["--self-test"] do
  ExUnit.start()
  defmodule RatesTest do
    use ExUnit.Case
    test "sorts dates and calculates variation with integer arithmetic" do
      rows = HistoricalRates.parse("data\tmoeda\tmicros\n2026-01-02\tUSD\t5500000\n2026-01-01\tUSD\t5000000\n")
      assert HistoricalRates.compare(rows,"USD").variation_basis_points == 1000
    end
    test "rejects duplicated observations" do
      assert_raise ArgumentError, fn -> HistoricalRates.parse("data\tmoeda\tmicros\n2026-01-01\tUSD\t1\n2026-01-01\tUSD\t2\n") end
    end
  end
else
  try do
    [file, currency] = System.argv()
    IO.inspect(HistoricalRates.compare(HistoricalRates.parse(File.read!(file)), currency), pretty: true)
  rescue
    error -> IO.puts(:stderr,"Erro: #{Exception.message(error)}. Uso: elixir main.exs cotacoes.tsv USD"); System.halt(2)
  end
end
