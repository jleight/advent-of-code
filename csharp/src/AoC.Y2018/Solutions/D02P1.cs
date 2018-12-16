using System;
using System.Linq;
using System.Threading.Tasks;
using AoC.Abstractions;

namespace AoC.Y2018.Solutions
{
    public class D02P1 : SolutionBase
    {
        public override Task Run()
        {
            var result = InputLines
                .Select(Parse)
                .Aggregate((a, e) => (a.Doubles + e.Doubles, a.Triples + e.Triples));

            Console.WriteLine(result.Doubles * result.Triples);
            return Task.CompletedTask;
        }

        private static (int Doubles, int Triples) Parse(string barcode)
        {
            var counts = barcode
                .GroupBy(c => c)
                .Select(g => g.Count())
                .Distinct()
                .ToList();
            var doubles = counts.Count(c => c == 2);
            var triples = counts.Count(c => c == 3);
            return (doubles, triples);
        }
    }
}
