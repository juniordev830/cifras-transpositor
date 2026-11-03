using Dates, Statistics
struct Observation
    date::Date
    station::String
    temperature::Union{Missing,Float64}
    rain::Union{Missing,Float64}
end
function parse_data(text)
    rows=split(chomp(text),'\n')
    strip(rows[1])=="data,estacao,temperatura,chuva" || error("Cabeçalho CSV inválido")
    observations=Observation[]; seen=Set{Tuple{Date,String}}()
    for row in rows[2:end]
        cells=split(strip(row),',';keepempty=true); length(cells)==4 || error("Esperadas 4 colunas, sem vírgulas nos campos")
        day=Date(cells[1],dateformat"yyyy-mm-dd"); station=String(cells[2]); isempty(station) && error("Estação vazia")
        key=(day,station); key in seen && error("Data/estação duplicada");push!(seen,key)
        temp=isempty(cells[3]) ? missing : parse(Float64,cells[3]); rain=isempty(cells[4]) ? missing : parse(Float64,cells[4])
        (ismissing(temp) || (isfinite(temp) && -90<=temp<=60)) || error("Temperatura inválida")
        (ismissing(rain) || (isfinite(rain) && rain>=0)) || error("Chuva inválida")
        push!(observations,Observation(day,station,temp,rain))
    end
    observations
end
function summarize(observations)
    groups=Dict{Tuple{String,Date},Vector{Observation}}()
    for row in observations
        push!(get!(groups,(row.station,firstdayofmonth(row.date)),Observation[]),row)
    end
    result=[]
    for ((station,month),rows) in sort(collect(groups);by=first)
        temperatures=collect(skipmissing([r.temperature for r in rows])); rains=collect(skipmissing([r.rain for r in rows]))
        push!(result,(station=station,month=month,expected_days=daysinmonth(month),temperature_days=length(temperatures),rain_days=length(rains),mean_temperature=isempty(temperatures) ? missing : mean(temperatures),observed_rain=isempty(rains) ? missing : sum(rains)))
    end
    result
end
try
    if ARGS==["--self-test"]
        rows=parse_data("data,estacao,temperatura,chuva\n2026-02-01,DEMO,30,0\n2026-02-02,DEMO,,5\n")
        summary=only(summarize(rows))
        @assert summary.expected_days==28 && summary.temperature_days==1 && summary.mean_temperature==30 && summary.observed_rain==5
        rejected=false
        try parse_data("data,estacao,temperatura,chuva\n2026-01-01,A,NaN,0") catch; rejected=true end
        @assert rejected
        println("2 cenários OK")
    else
        length(ARGS)==1 || error("Uso: julia main.jl observacoes.csv")
        for row in summarize(parse_data(read(ARGS[1],String))); println(row); end
    end
catch error_value
    println(stderr,"Erro: ",sprint(showerror,error_value));exit(2)
end
