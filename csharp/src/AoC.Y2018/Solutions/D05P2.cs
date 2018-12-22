using System;
using System.Linq;
using System.Threading.Tasks;
using AoC.Abstractions;

namespace AoC.Y2018.Solutions
{
    public class D05P2 : SolutionBase
    {
        public override Task Run()
        {
            var result = InputString
                .ToUpperInvariant()
                .Distinct()
                .ToList()
                .Select(unit => Replace(unit, InputString))
                .Select(D05P1.React)
                .Min(polymer => polymer.Length);

            Console.WriteLine(result);
            return Task.CompletedTask;
        }

        private static string Replace(char unit, string polymer)
        {
            return polymer
                .Replace(unit.ToString(), string.Empty)
                .Replace(char.ToLowerInvariant(unit).ToString(), string.Empty);
        }
    }
}
