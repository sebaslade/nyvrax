declare const router: {
  get(...args: unknown[]): void;
};

declare const requireAuth: unknown;
declare const getOrder: unknown;

router.get(
  "/orders/:id",
  requireAuth,
  getOrder,
);