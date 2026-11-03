//> using scala "3.3.5"
import scala.io.Source
object Main {
  case class Accepted(id: String, age: Int, email: String)
  case class Rejected(line: Int, reasons: List[String])
  def validate(rows: List[String]): (List[Accepted], List[Rejected]) = {
    require(rows.headOption.contains("id\tidade\temail"), "Cabeçalho TSV inválido")
    val seen=scala.collection.mutable.Set.empty[String]
    val good=List.newBuilder[Accepted]; val bad=List.newBuilder[Rejected]
    rows.drop(1).zipWithIndex.foreach { case (row,index) =>
      val fields=row.split("\t",-1)
      if(fields.length!=3) bad+=Rejected(index+2,List("colunas"))
      else {
        val id=fields(0); val age=fields(1).toIntOption
        val reasons=List(
          Option.when(id.isEmpty)("id_vazio"),
          Option.when(id.nonEmpty && seen.contains(id))("id_duplicado"),
          Option.when(age.forall(x=>x<0 || x>130))("idade_invalida"),
          Option.when(!fields(2).matches("[^\\s@]+@[^\\s@]+\\.[^\\s@]+"))("email_invalido")
        ).flatten
        if(id.nonEmpty) seen+=id
        if(reasons.isEmpty) good+=Accepted(id,age.get,fields(2)) else bad+=Rejected(index+2,reasons)
      }
    }
    (good.result(),bad.result())
  }
  def main(args: Array[String]): Unit = try {
    if(args.sameElements(Array("--self-test"))) {
      val (good,bad)=validate(List("id\tidade\temail","001\t26\ta@demo.test","001\t-1\tb@demo.test","002\tx\tbad"))
      require(good.map(_.id)==List("001") && bad.length==2 && bad.head.reasons.contains("id_duplicado"))
      println("3 cenários OK")
    } else {
      require(args.length==1,"Uso: scala Main.scala -- entrada.tsv")
      val source=Source.fromFile(args(0),"UTF-8")
      val result=try validate(source.getLines().toList) finally source.close()
      result._1.foreach(x=>println(s"ACEITO\t${x.id}\t${x.age}\t${x.email}"))
      result._2.foreach(x=>println(s"REJEITADO\tlinha=${x.line}\t${x.reasons.mkString(",")}"))
    }
  } catch {case e: Exception=>System.err.println("Erro: "+e.getMessage);sys.exit(2)}
}
