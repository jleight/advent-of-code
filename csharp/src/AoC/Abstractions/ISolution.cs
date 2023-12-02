namespace AoC.Abstractions;

[UsedImplicitly(ImplicitUseTargetFlags.WithInheritors)]
public interface ISolution
{
    Task Run(SolutionContext context);
}
