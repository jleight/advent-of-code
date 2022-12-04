namespace AoC.Abstractions;

public interface ISolution
{
    Task Run(SolutionContext context);
}
