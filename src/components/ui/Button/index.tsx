import { type FC, ReactNode } from 'react';
import { ButtonOwnProps, Button as MuiButton } from '@mui/material';

type ButtonVariants = 'primary' | 'secondary' | 'ghost';

interface ButtonProps {
  children: ReactNode;
  className?: string;
  variant?: ButtonVariants;
  size?: ButtonOwnProps['size'];
  onClick?: () => void;
}

const Button: FC<ButtonProps> = ({
  children,
  className,
  size = 'medium',
  variant = 'primary',
  onClick
}) => {
  const getVariant = () => {
    switch (variant) {
      case 'primary':
        return 'contained';
      case 'secondary':
        return 'outlined';
      case 'ghost':
        return 'text';
      default:
        return 'contained';
    }
  };

  return (
    <MuiButton
      className={className}
      size={size}
      onClick={onClick}
      variant={getVariant()}>
      {children}
    </MuiButton>
  );
};

export default Button;
