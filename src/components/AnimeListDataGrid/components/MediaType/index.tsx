interface MediaTypeProps {
  mediaType: string;
}

const MediaType = ({ mediaType }: MediaTypeProps) => {
  return (
    <div className="flex justify-center text-sm w-full">
      {mediaType !== 'Unknown' ? (
        mediaType
      ) : (
        <span className="text-gray-400">Unknown</span>
      )}
    </div>
  );
};

export default MediaType;
