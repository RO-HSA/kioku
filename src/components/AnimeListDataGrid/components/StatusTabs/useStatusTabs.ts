import { useEffect, useRef } from 'react';

const LONG_PRESS_MS = 80;
const MOVE_THRESHOLD = 8;

const useStatusTabs = () => {
  const tabsRootRef = useRef<HTMLDivElement | null>(null);
  const dragStateRef = useRef({
    isDown: false,
    hasMoved: false,
    startX: 0,
    startY: 0,
    scrollLeft: 0,
    canDrag: false,
    longPress: false,
    longPressTimer: null as number | null
  });

  useEffect(() => {
    const root = tabsRootRef.current;

    if (!root) return;

    const scroller = root.querySelector(
      '.MuiTabs-scroller'
    ) as HTMLDivElement | null;

    if (!scroller) return;

    const onPointerDown = (event: PointerEvent) => {
      if (event.pointerType !== 'mouse') return;
      if (event.button !== 0) return;

      dragStateRef.current.isDown = true;
      dragStateRef.current.hasMoved = false;
      dragStateRef.current.startX = event.clientX;
      dragStateRef.current.startY = event.clientY;
      dragStateRef.current.scrollLeft = scroller.scrollLeft;
      dragStateRef.current.canDrag =
        scroller.scrollWidth > scroller.clientWidth + 1;
      dragStateRef.current.longPress = false;

      if (dragStateRef.current.longPressTimer) {
        window.clearTimeout(dragStateRef.current.longPressTimer);
      }
      if (dragStateRef.current.canDrag) {
        dragStateRef.current.longPressTimer = window.setTimeout(() => {
          dragStateRef.current.longPress = true;
        }, LONG_PRESS_MS);
      }
    };

    const onPointerMove = (event: PointerEvent) => {
      const state = dragStateRef.current;

      if (!state.isDown || !state.canDrag) return;
      if (event.pointerType !== 'mouse') return;
      if (!state.longPress) return;

      const dx = event.clientX - state.startX;
      const dy = event.clientY - state.startY;

      if (
        !state.hasMoved &&
        Math.abs(dx) > MOVE_THRESHOLD &&
        Math.abs(dx) > Math.abs(dy)
      ) {
        state.hasMoved = true;
        scroller.setPointerCapture(event.pointerId);
        scroller.dataset.dragging = 'true';
      }

      if (state.hasMoved) {
        scroller.scrollLeft = state.scrollLeft - dx;
        event.preventDefault();
      }
    };

    const endDrag = (event: PointerEvent) => {
      const state = dragStateRef.current;

      if (!state.isDown) return;

      state.isDown = false;
      state.canDrag = false;
      state.longPress = false;
      if (state.longPressTimer) {
        window.clearTimeout(state.longPressTimer);
        state.longPressTimer = null;
      }

      if (state.hasMoved) {
        delete scroller.dataset.dragging;
      }

      if (scroller.hasPointerCapture(event.pointerId)) {
        scroller.releasePointerCapture(event.pointerId);
      }
    };

    const onClick = (event: MouseEvent) => {
      if (dragStateRef.current.hasMoved || dragStateRef.current.longPress) {
        event.preventDefault();
        event.stopPropagation();
        dragStateRef.current.hasMoved = false;
        dragStateRef.current.longPress = false;
      }
    };

    scroller.addEventListener('pointerdown', onPointerDown);
    scroller.addEventListener('pointermove', onPointerMove);
    scroller.addEventListener('pointerup', endDrag);
    scroller.addEventListener('pointerleave', endDrag);
    scroller.addEventListener('pointercancel', endDrag);
    scroller.addEventListener('click', onClick, true);

    return () => {
      scroller.removeEventListener('pointerdown', onPointerDown);
      scroller.removeEventListener('pointermove', onPointerMove);
      scroller.removeEventListener('pointerup', endDrag);
      scroller.removeEventListener('pointerleave', endDrag);
      scroller.removeEventListener('pointercancel', endDrag);
      scroller.removeEventListener('click', onClick, true);
    };
  }, []);

  return { tabsRootRef };
};

export default useStatusTabs;
