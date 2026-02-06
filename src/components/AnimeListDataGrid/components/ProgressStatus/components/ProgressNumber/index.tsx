import { FC } from 'react';

export interface ProgressNumberProps {
  progress: number;
  total: number;
}

const ProgressNumber: FC<ProgressNumberProps> = ({ progress, total }) => {
  return (
    <div className="text-sm">
      {progress}/
      <span className={total === 0 ? 'text-gray-400' : ''}>{total || '?'}</span>
    </div>
  );
};

export default ProgressNumber;
