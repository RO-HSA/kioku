import { ButtonOwnProps, Button as MuiButton } from '@mui/material';
import { type FC, ReactNode } from 'react';

type ButtonVariants = 'primary' | 'secondary' | 'ghost';

interface ButtonProps {
  children: ReactNode;
  className?: string;
  type?: 'button' | 'submit' | 'reset';
  isDisabled?: boolean;
  isLoading?: boolean;
  variant?: ButtonVariants;
  size?: ButtonOwnProps['size'];
  onClick?: () => void;
}

const Button: FC<ButtonProps> = ({
  children,
  className,
  isDisabled = false,
  isLoading = false,
  type = 'button',
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
      type={type}
      disabled={isDisabled}
      loading={isLoading}
      style={{ minWidth: '32px' }}
      onClick={onClick}
      variant={getVariant()}>
      {children}
    </MuiButton>
  );
};

export default Button;
