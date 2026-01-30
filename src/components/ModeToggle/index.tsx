import { useColorScheme } from '@mui/material';
import { ChangeEvent } from 'react';

import { useSidebarStore } from '@/stores/sidebar/sidebar';
import { MoonStar, Sun } from 'lucide-react';
import Button from '../ui/Button';
import ModeSwitch from './components/ModeSwitch';

const ModeToggle = () => {
  const isSidebarOpen = useSidebarStore((state) => state.isOpen);
  const { mode, setMode } = useColorScheme();

  const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
    setMode(event.target.checked ? 'dark' : 'light');
  };

  if (!isSidebarOpen) {
    return (
      <Button
        onClick={() => setMode(mode === 'dark' ? 'light' : 'dark')}
        variant="ghost">
        {mode === 'dark' ? (
          <MoonStar className="size-5" />
        ) : (
          <Sun className="size-5" />
        )}
      </Button>
    );
  }

  return <ModeSwitch checked={mode === 'dark'} onChange={handleChange} />;
};

export default ModeToggle;
