export const formatElapsedTime = (start: Date, end: Date): string => {
	const diff = end.getTime() - start.getTime();
	return formatTimeDiff(diff);
};

const formatTimeDiff = (diff: number, useNanoSeconds: boolean = false): string => {
	const hours = Math.floor(diff / 3600000);
	const minutes = Math.floor((diff % 3600000) / 60000);
	const seconds = Math.floor((diff % 60000) / 1000);
	const milliseconds = diff % 1000;

	const formattedResult = `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;

	if (useNanoSeconds) {
		return `${formattedResult}.${milliseconds.toString().padStart(3, '0')}`;
	}

	return formattedResult;
};
