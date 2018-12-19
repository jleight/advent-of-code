using System;
using System.Linq;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using AoC.Abstractions;

namespace AoC.Y2018.Solutions
{
    public class D03P2 : SolutionBase
    {
        private readonly static Regex _parser = new Regex(@"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)");

        public override Task Run()
        {
            var claims = InputLines
                .Select(Parse)
                .ToList();

            var maxX = claims.Max(c => c.X + c.DX);
            var maxY = claims.Max(c => c.Y + c.DY);

            var grid = new bool?[maxX, maxY];

            foreach (var claim in claims)
            {
                for (var x = claim.X; x < claim.X + claim.DX; x++)
                for (var y = claim.Y; y < claim.Y + claim.DY; y++)
                    grid[x, y] = grid[x, y] == null ? false : true;
            }

            foreach (var claim in claims)
            {
                var allFalse = true;

                for (var x = claim.X; x < claim.X + claim.DX; x++)
                for (var y = claim.Y; y < claim.Y + claim.DY; y++)
                    if (grid[x, y] == true)
                        allFalse = false;

                if (allFalse)
                {
                    Console.WriteLine(claim.Id);
                    return Task.CompletedTask;
                }
            }

            Console.WriteLine("All claims have overlap!");
            return Task.CompletedTask;
        }

        private static (int Id, int X, int Y, int DX, int DY) Parse(string claim)
        {
            var match = _parser.Match(claim);
            return (
                Convert.ToInt32(match.Groups[1].Value, 10),
                Convert.ToInt32(match.Groups[2].Value, 10),
                Convert.ToInt32(match.Groups[3].Value, 10),
                Convert.ToInt32(match.Groups[4].Value, 10),
                Convert.ToInt32(match.Groups[5].Value, 10)
            );
        }
    }
}