using System;
using System.Linq;
using System.Threading.Tasks;
using AoC.Abstractions;

namespace AoC.Y2018.Solutions
{
    public class D05P1 : SolutionBase
    {
        public override Task Run()
        {
            Console.WriteLine(React(InputString).Length);
            return Task.CompletedTask;
        }

        internal static string React(string polymer)
        {
            var modified = false;

            do
            {
                modified = false;
                for (var i = 0; i < polymer.Length - 1; i++)
                {
                    var unit = polymer[i];
                    var opposite = char.IsLower(unit)
                        ? char.ToUpperInvariant(unit)
                        : char.ToLowerInvariant(unit);

                    var nextUnit = polymer[i + 1];

                    if (nextUnit != opposite)
                        continue;

                    polymer = polymer.Remove(i, 2);
                    modified = true;
                    i -= 1;
                }
            } while (modified);

            return polymer;
        }
    }
}
