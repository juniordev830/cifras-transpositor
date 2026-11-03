import 'dart:convert';
import 'dart:io';
void expect(bool value) { if (!value) throw StateError('Teste falhou'); }
DateTime date(String value) {
  final parsed=DateTime.parse(value);
  if (!RegExp(r'^\d{4}-\d{2}-\d{2}$').hasMatch(value) || parsed.toIso8601String().substring(0,10)!=value) throw FormatException('Data inválida');
  return DateTime.utc(parsed.year,parsed.month,parsed.day);
}

Map<String,dynamic> inventory(List<dynamic> rows) {
 final ids=<String>{}; final rooms=<String,Map<String,int>>{};
 for(final raw in rows) {
  final row=raw as Map<String,dynamic>;
  final id=row['id'], room=row['comodo'], quantity=row['quantidade'], cents=row['valor_unitario_centavos'];
  if(id is! String || id.trim().isEmpty || !ids.add(id) || room is! String || room.trim().isEmpty || quantity is! int || quantity<=0 || cents is! int || cents<0) throw FormatException('Item inválido ou ID repetido');
  final amount=quantity*cents;
  if(amount>9007199254740991) throw FormatException('Valor fora do intervalo suportado');
  final group=rooms.putIfAbsent(room,()=>{'unidades':0,'centavos':0});
  group['unidades']=group['unidades']!+quantity; group['centavos']=group['centavos']!+amount;
  if(group['centavos']!>9007199254740991) throw FormatException('Total excessivo');
 }
 return {'por_comodo':rooms,'total_centavos':rooms.values.fold<int>(0,(sum,x)=>sum+x['centavos']!)};
}
void main(List<String> args) {
 try {
  if(args.length==1 && args[0]=='--self-test') {
   final item={'id':'x','comodo':'Sala','quantidade':2,'valor_unitario_centavos':150};
   expect(inventory([item])['total_centavos']==300);
   var bad=false; try {inventory([item,item]);} catch(_){bad=true;} expect(bad);
   print('2 cenários OK'); return;
  }
  if(args.length!=1) throw FormatException('Uso: dart run bin/main.dart itens.json');
  print(const JsonEncoder.withIndent('  ').convert(inventory(jsonDecode(File(args[0]).readAsStringSync()) as List<dynamic>)));
 } catch(e) {stderr.writeln('Erro: $e'); exitCode=2;}
}
