defmodule MinistryRoster do
  def parse(text) do
    [header | rows] = String.split(text, "\n", trim: true)
    if String.trim_trailing(header,"\r") != "nome\tinstrumentos\tdata", do: raise(ArgumentError,"Cabeçalho TSV inválido")
    members = Enum.map(rows, fn row ->
      [name, skills, date] = String.split(String.trim_trailing(row,"\r"), "\t")
      {:ok, day} = Date.from_iso8601(date)
      if String.trim(name)=="" or String.trim(skills)=="", do: raise(ArgumentError,"Participante inválido")
      %{name: name, skills: String.split(skills, ",", trim: true), date: day}
    end)
    if length(members)>20, do: raise(ArgumentError,"Limite de 20 registros")
    members
  end
  def assign(members, roles, date) do
    if roles==[] or length(roles)>7 or length(Enum.uniq(roles))!=length(roles), do: raise(ArgumentError,"Informe de 1 a 7 funções distintas")
    roles
    |> Enum.sort_by(fn role -> Enum.count(members, &(&1.date==date and role in &1.skills)) end)
    |> find(members, date, MapSet.new(), %{})
  end
  defp find([], _members, _date, _used, result), do: {:ok, result}
  defp find([role | rest], members, date, used, result) do
    members |> Enum.filter(&(&1.date==date and role in &1.skills and not MapSet.member?(used,&1.name)))
    |> Enum.sort_by(& &1.name)
    |> Enum.reduce_while({:error,:unavailable}, fn member, _ ->
      case find(rest,members,date,MapSet.put(used,member.name),Map.put(result,role,member.name)) do
        {:ok, _}=found -> {:halt, found}
        error -> {:cont, error}
      end
    end)
  end
end
if System.argv()==["--self-test"] do
  ExUnit.start()
  defmodule RosterTest do
    use ExUnit.Case
    test "never assigns the same person twice and handles scarce skills" do
      date = ~D[2026-11-08]
      members=[%{name: "Ana",skills: ["voz","violao"],date: date},%{name: "Bia",skills: ["voz"],date: date}]
      assert MinistryRoster.assign(members,["voz","violao"],date)=={:ok,%{"voz"=>"Bia","violao"=>"Ana"}}
      assert MinistryRoster.assign(members,["baixo"],date)=={:error,:unavailable}
    end
  end
else
  try do
    [file, day, roles]=System.argv(); {:ok,date}=Date.from_iso8601(day)
    IO.inspect(MinistryRoster.assign(MinistryRoster.parse(File.read!(file)),String.split(roles,","),date))
  rescue
    error -> IO.puts(:stderr,"Erro: #{Exception.message(error)}. Uso: elixir main.exs equipe.tsv DATA voz,violao");System.halt(2)
  end
end
