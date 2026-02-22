import { useConfigMenuStore } from '@/stores/config/configMenu';
import { ConfigMenuStep } from '@/types/Navigation';
import Application from '../Application';
import Detection from '../Detection';
import Integrations from '../Integrations';
import Sharing from '../Sharing';

const MenuContent = () => {
  const step = useConfigMenuStore((state) => state.step);

  const getStepContent = (step: ConfigMenuStep) => {
    switch (step) {
      case ConfigMenuStep.INTEGRATIONS:
        return <Integrations />;
      case ConfigMenuStep.DETECTION:
        return <Detection />;
      case ConfigMenuStep.APPLICATION:
        return <Application />;
      case ConfigMenuStep.SHARING:
        return <Sharing />;
      default:
        return null;
    }
  };

  return <div className="px-3 pt-2 sm:pt-0 w-full">{getStepContent(step)}</div>;
};

export default MenuContent;
