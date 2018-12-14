using System;
using System.Linq;
using System.Threading.Tasks;
using AoC.Abstractions;

namespace AoC
{
    public static class Solution
    {
        public static async Task Run(Type assemblyType, string name)
        {
            var solutionType = assemblyType
                .Assembly
                .DefinedTypes
                .Where(t => typeof(ISolution).IsAssignableFrom(t) && !t.IsAbstract && !t.IsInterface && t.IsPublic)
                .FirstOrDefault(t => t.Name.Equals(name, StringComparison.OrdinalIgnoreCase));

            if (solutionType is null)
            {
                await Console.Error.WriteLineAsync("Solution not implemented!");
                return;
            }

            var solution = Activator.CreateInstance(solutionType) as ISolution;
            if (solution is null)
            {
                await Console.Error.WriteLineAsync("Solution not an ISolution!");
                return;
            }

            await solution.Run();
        }
    }
}
