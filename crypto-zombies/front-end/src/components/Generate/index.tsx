import React from 'react';
import ZombieGene from './ZombieGene';
import ZombiePreview from './ZombiePreview';

function Generate() {
  const background = ' https://cryptozombies.io/course/static/img/walls.jpg';
  return (
    <div
      className="w-screen min-h-screen object-cover bg-cover text-[#fff] flex "
      style={{ backgroundImage: `url(${background})`, width: '100vw', height: '100vh', backgroundSize: 'cover' }}
    >
      {/* zombie - preview */}
      <ZombiePreview />
      {/* zombie- toggle */}
      <ZombieGene />
    </div>
  );
}

export default Generate;
