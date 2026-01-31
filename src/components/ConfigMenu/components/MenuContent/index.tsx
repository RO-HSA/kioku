import { useConfigMenuStore } from '@/stores/config/configMenu';
import { ConfigMenuStep } from '@/types/Navigation';
import Integrations from '../Integrations';

const MenuContent = () => {
  const step = useConfigMenuStore((state) => state.step);

  const getStepContent = (step: ConfigMenuStep) => {
    switch (step) {
      case ConfigMenuStep.INTEGRATIONS:
        return <Integrations />;
      default:
        return null;
    }
  };

  return <div className="px-3 pt-2 sm:pt-0">{getStepContent(step)}</div>;
};

export default MenuContent;
