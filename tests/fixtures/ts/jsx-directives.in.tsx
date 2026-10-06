const el = (
  <div>
    {/* eslint-disable-next-line react/no-danger */}
    <p dangerouslySetInnerHTML={{ __html: html }} />
    {/* SAFETY: x */}
    {/* drop */}
  </div>
);
