import { useConfigMenuStore } from '@/stores/config/configMenu';
import { ConfigMenuStep } from '@/types/Navigation';
import Detection from '../Detection';
import Integrations from '../Integrations';
import Updates from '../Updates';

const MenuContent = () => {
  const step = useConfigMenuStore((state) => state.step);

  const getStepContent = (step: ConfigMenuStep) => {
    switch (step) {
      case ConfigMenuStep.INTEGRATIONS:
        return <Integrations />;
      case ConfigMenuStep.DETECTION:
        return <Detection />;
      case ConfigMenuStep.UPDATES:
        return <Updates />;
      default:
        return null;
    }
  };

  return <div className="px-3 pt-2 sm:pt-0 w-full">{getStepContent(step)}</div>;
};

export default MenuContent;
