defmodule WebhookInbox do
  use GenServer
  def start_link, do: GenServer.start_link(__MODULE__, nil)
  def add(pid, headers, body), do: GenServer.call(pid, {:add, headers, body})
  def list(pid), do: GenServer.call(pid, :list)
  @impl true
  def init(_), do: {:ok, %{next_id: 1, events: []}}
  @impl true
  def handle_call(:list, _from, state) do
    state=prune(state); {:reply,Enum.reverse(state.events),state}
  end
  def handle_call({:add, headers, body}, _from, state) do
    state=prune(state)
    valid = is_map(headers) and map_size(headers)<=30 and is_binary(body) and byte_size(body)<=16_384 and Enum.all?(headers, fn {key,value} -> is_binary(key) and is_binary(value) and byte_size(key)+byte_size(value)<=1024 end)
    if valid do
      clean=Map.new(headers,fn {key,value} ->
        name=String.downcase(key)
        {name, if(name in ["authorization","cookie","x-api-key"], do: "[OCULTO]", else: value)}
      end)
      event=%{id: state.next_id,headers: clean,body: body,received_at: System.monotonic_time(:millisecond)}
      {:reply,{:ok,event.id},%{state|next_id: state.next_id+1,events: Enum.take([event|state.events],50)}}
    else
      {:reply,{:error,:invalid_or_too_large},state}
    end
  end
  defp prune(state) do
    cutoff=System.monotonic_time(:millisecond)-600_000
    %{state|events: Enum.filter(state.events,&(&1.received_at>=cutoff))}
  end
end
if System.argv()==["--self-test"] do
  ExUnit.start()
  defmodule InboxTest do
    use ExUnit.Case
    test "masks selected headers, bounds body and retains at most 50 events" do
      {:ok,pid}=WebhookInbox.start_link()
      assert WebhookInbox.add(pid,%{"Authorization"=>"dummy"},"hello")=={:ok,1}
      assert hd(WebhookInbox.list(pid)).headers["authorization"]=="[OCULTO]"
      assert WebhookInbox.add(pid,%{},String.duplicate("x",16_385))=={:error,:invalid_or_too_large}
      for _ <- 1..60, do: WebhookInbox.add(pid,%{},"demo")
      assert length(WebhookInbox.list(pid))==50
    end
  end
else
  if System.argv()==["--demo"] do
    {:ok,pid}=WebhookInbox.start_link()
    WebhookInbox.add(pid,%{"Content-Type"=>"text/plain","Authorization"=>"token-ficticio"},"evento de desenvolvimento")
    IO.inspect(WebhookInbox.list(pid),pretty: true)
  else
    IO.puts(:stderr,"Uso: elixir main.exs --demo | --self-test");System.halt(2)
  end
end
