using Xunit;

namespace RomesteadTrainer.Tests
{
    public class TrainerTests
    {
        [Fact]
        public void GainExperience_AddsXP()
        {
            var trainer = new Trainer();
            trainer.GainExperience(50);
            Assert.Equal(50, trainer.GetExperience());
        }

        [Fact]
        public void GainExperience_LevelsUpAtThreshold()
        {
            var trainer = new Trainer();
            for (int i = 0; i < 10; i++)
                trainer.GainExperience(100);
            Assert.Equal(2, trainer.GetLevel());
            Assert.Equal(0, trainer.GetExperience());
        }

        [Fact]
        public void InitialState_IsLevelOne()
        {
            var trainer = new Trainer();
            Assert.Equal(1, trainer.GetLevel());
            Assert.Equal(0, trainer.GetExperience());
        }
    }
}